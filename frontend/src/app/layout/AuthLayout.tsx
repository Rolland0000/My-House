import { Outlet } from "react-router";
import { Card } from "../../shared/components";

export function AuthLayout() {
  return (
    <div className="flex min-h-screen justify-center bg-bg px-4 py-12">
      <div className="flex w-full max-w-md flex-col gap-8">
        <p className="text-center text-sm font-bold tracking-widest text-primary uppercase">
          MyHouse
        </p>
        <Card>
          <Outlet />
        </Card>
      </div>
    </div>
  );
}
