import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "Uselytics - Realtime open source analytics",
  description:
    " Gain instant insights into user behavior with Uselytics, a free, open-source real-time analytics platform.",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body
        className={`${inter.className} bg-stone-50 font-sans text-stone-900`}
      >
        {children}
      </body>
    </html>
  );
}
