"use server";

import { env } from "@/env";
import { z } from "zod";
import { revalidatePath } from "next/cache";

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
      console.log({ parsed });
      revalidatePath("/");
      return {
        message: "🎉 You're successfully added to the waitlist!",
      };
    } catch (error) {
      console.log(error);
      return { message: "❌ Something went wrong, please try again!" };
    }
  } else {
    return { message: "❌ Please enter a valid email!" };
  }
}
