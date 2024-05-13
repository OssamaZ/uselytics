"use client";

import { useFormState, useFormStatus } from "react-dom";
import { SignupForWaitlist } from "./action";

function SubmitButton() {
  const { pending } = useFormStatus();

  return (
    <button
      className="text-white inline-flex items-center justify-center transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 rounded-md px-8 mt-5 h-12 w-full bg-gradient-to-bl from-[#5c5aef] to-[#1b18ca] text-lg font-normal md:ml-5 md:mt-0 md:min-w-[170px]"
      aria-disabled={pending}
      disabled={pending}
    >
      {pending ? "Joining .." : "Join waitlist"}
    </button>
  );
}

export default function WaitlistForm() {
  const [state, formAction] = useFormState(SignupForWaitlist, {
    message: "",
  });

  return (
    <form className="flex flex-col gap-3" action={formAction}>
      <p className="text-center font-medium text-stone-700">
        Follow our journey and join the waitlist now.
      </p>
      <div
        suppressHydrationWarning={true}
        className="flex flex-col items-center justify-center rounded-[10px] md:flex-row"
      >
        <input
          className="relative h-12 w-full rounded-md border border-stone-200 bg-white/80 text-base md:min-w-[300px]"
          name="email"
          required
          type="email"
          placeholder="Your email"
        />
        <SubmitButton />
      </div>
      {state.message ? <p className="text-center">{state.message}</p> : null}
    </form>
  );
}
