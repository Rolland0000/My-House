import { Link, Outlet } from "react-router";
import { Card } from "../../shared/components";

export function AuthLayout() {
  return (
    <div className="flex min-h-screen flex-col bg-bg">
      <div className="flex h-14 items-center border-b border-border bg-surface px-6">
        <Link to="/" className="flex items-baseline gap-1.5">
          <span className="text-[17px] font-extrabold tracking-tight text-ink-900">MY HOUSE</span>
          <span className="size-1.25 bg-primary" aria-hidden="true" />
        </Link>
      </div>
      <div className="flex flex-1 justify-center px-4 py-8">
        <div className="w-full max-w-md">
          <Card>
            <Outlet />
          </Card>
        </div>
      </div>
    </div>
  );
}
