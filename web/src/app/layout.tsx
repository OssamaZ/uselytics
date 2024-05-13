import type { Metadata } from "next";
import { Inter as FontSans } from "next/font/google";
import "@/styles/globals.css";

import { cn } from "@/lib/utils";

const fontSans = FontSans({
  subsets: ["latin"],
  variable: "--font-sans",
});

export const metadata: Metadata = {
  title: "Uselytics - Realtime open source analytics",
  description:
    "Gain instant insights into user behavior with Uselytics, a free, open-source real-time analytics platform.",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={cn("bg-muted font-sans antialiased", fontSans.variable)}>
        {children}
      </body>
    </html>
  );
}
