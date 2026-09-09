import type { HTMLAttributes } from "react";
import { CheckCircle2, Info, TriangleAlert, CircleX } from "lucide-react";
import { cn } from "../utils/cn";

type AlertVariant = "success" | "warning" | "error" | "info";

interface AlertProps extends HTMLAttributes<HTMLDivElement> {
  variant?: AlertVariant;
  title?: string;
}

const variantConfig: Record<
  AlertVariant,
  { icon: typeof Info; classes: string; iconClasses: string }
> = {
  success: {
    icon: CheckCircle2,
    classes: "border-success/30 bg-success-soft text-text",
    iconClasses: "text-success",
  },
  warning: {
    icon: TriangleAlert,
    classes: "border-warning/35 bg-warning-soft text-text",
    iconClasses: "text-warning-text",
  },
  error: {
    icon: CircleX,
    classes: "border-error/35 bg-error-soft text-text",
    iconClasses: "text-error-text",
  },
  info: {
    icon: Info,
    classes: "border-ink-600/30 bg-primary-soft text-text",
    iconClasses: "text-ink-600",
  },
};

function Alert({ variant = "info", title, className, children, ...rest }: AlertProps) {
  const { icon: Icon, classes, iconClasses } = variantConfig[variant];

  return (
    <div
      role="alert"
      className={cn("flex gap-3 rounded-sm border px-4 py-3 text-sm", classes, className)}
      {...rest}
    >
      <Icon className={cn("size-5 shrink-0", iconClasses)} aria-hidden="true" />
      <div className="flex flex-col gap-0.5 text-text">
        {title && <p className="font-semibold">{title}</p>}
        {children && <div>{children}</div>}
      </div>
    </div>
  );
}

export { Alert };
export type { AlertProps, AlertVariant };
