import { Button } from "@/components/ui/button";
import { auth, signIn } from "@/lib/auth";

export default async function Dashboard() {
  const session = await auth();
  if (!session)
    return (
      <div>
        <p>Not authenticated</p>
        <form
          action={async () => {
            "use server";
            await signIn();
          }}
        >
          <Button type="submit">Sign in</Button>
        </form>
      </div>
    );
  const user = session.user?.name;
  return (
    <p className="flex gap-1">
      <span>Hello</span>
      <span className="font-medium">{user}!</span>
    </p>
  );
}
