import NextAuth from "next-auth";
import { PrismaAdapter } from "@auth/prisma-adapter";
import google from "next-auth/providers/google";

import { db } from "@/lib/db";

export const { handlers, auth, signIn, signOut } = NextAuth({
  adapter: PrismaAdapter(db),
  providers: [google],
  trustHost: true,
});
