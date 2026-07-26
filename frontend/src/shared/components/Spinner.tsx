import { LoaderCircle } from "lucide-react";
import { cn } from "../utils/cn";

interface SpinnerProps {
  size?: "sm" | "md" | "lg";
  className?: string;
  label?: string;
}

const sizeClasses: Record<NonNullable<SpinnerProps["size"]>, string> = {
  sm: "size-4",
  md: "size-5",
  lg: "size-8",
};

function Spinner({ size = "md", className, label = "Chargement…" }: SpinnerProps) {
  return (
    <span role="status" className="inline-flex items-center">
      <LoaderCircle
        aria-hidden="true"
        className={cn("animate-spin text-current", sizeClasses[size], className)}
      />
      <span className="sr-only">{label}</span>
    </span>
  );
}

export { Spinner };
export type { SpinnerProps };
