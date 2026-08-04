import type { HTMLAttributes } from "react";
import { CheckCircle2, Info, TriangleAlert, CircleX } from "lucide-react";
import { cn } from "../utils/cn";

type AlertVariant = "success" | "warning" | "error" | "info";

interface AlertProps extends HTMLAttributes<HTMLDivElement> {
  variant?: AlertVariant;
  title?: string;
}

const variantConfig: Record<AlertVariant, { icon: typeof Info; classes: string }> = {
  success: { icon: CheckCircle2, classes: "border-success bg-success-soft text-success" },
  warning: { icon: TriangleAlert, classes: "border-warning bg-warning-soft text-warning" },
  error: { icon: CircleX, classes: "border-error bg-error-soft text-error" },
  info: { icon: Info, classes: "border-focus bg-primary-soft text-text" },
};

function Alert({ variant = "info", title, className, children, ...rest }: AlertProps) {
  const { icon: Icon, classes } = variantConfig[variant];

  return (
    <div
      role="alert"
      className={cn("flex gap-3 rounded-md border px-4 py-3 text-sm", classes, className)}
      {...rest}
    >
      <Icon className="size-5 shrink-0" aria-hidden="true" />
      <div className="flex flex-col gap-0.5 text-text">
        {title && <p className="font-semibold">{title}</p>}
        {children && <div>{children}</div>}
      </div>
    </div>
  );
}

export { Alert };
export type { AlertProps, AlertVariant };
