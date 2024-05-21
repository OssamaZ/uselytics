"use server";

import { z } from "zod";
import { revalidatePath } from "next/cache";
import { db } from "@/lib/db";
import { env } from "@/env";
import { Resend } from "resend";
import WaitlistEmail from "@/components/emails/waitlist";

const resend = new Resend(env.RESEND_KEY);

const schema = z.object({
  email: z.string().email(),
});

export async function SignupForWaitlist(
  prevState: { message: string },
  formData: FormData
) {
  const parsed = schema.safeParse({
    email: formData.get("email"),
  });

  if (parsed.success) {
    try {
      const email = parsed.data.email;
      const data = await db.waitlist.upsert({
        create: {
          email,
        },
        update: {},
        where: {
          email,
        },
      });
      const _ = await resend.emails.send({
        from: "Ossama <hello@uselytics.app>",
        to: [email],
        subject: "Welcome to Uselytics",
        react: WaitlistEmail({ email }),
      });
      if (data) {
        revalidatePath("/");
        return {
          message: "🎉 You're successfully added to the waitlist!",
        };
      }
      throw new Error("Unable to save in the database");
    } catch (error) {
      console.log(error);
      return { message: "❌ Something went wrong, please try again!" };
    }
  } else {
    return { message: "❌ Please enter a valid email!" };
  }
}
