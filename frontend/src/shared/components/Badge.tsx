import type { HTMLAttributes } from "react";
import { cn } from "../utils/cn";

type BadgeTone = "neutral" | "success" | "warning" | "error" | "brass";
type BadgeSize = "sm" | "md";

interface BadgeProps extends HTMLAttributes<HTMLSpanElement> {
  tone: BadgeTone;
  dot?: boolean;
  size?: BadgeSize;
}

const toneClasses: Record<BadgeTone, { wrap: string; dot: string }> = {
  neutral: { wrap: "border-border bg-surface text-text", dot: "bg-ink-500" },
  success: { wrap: "border-success/25 bg-success-soft text-success", dot: "bg-success" },
  warning: { wrap: "border-warning/30 bg-warning-soft text-warning-text", dot: "bg-warning" },
  error: { wrap: "border-error/30 bg-error-soft text-error-text", dot: "bg-error" },
  brass: { wrap: "border-border bg-surface text-ink-900", dot: "bg-primary" },
};

const sizeClasses: Record<BadgeSize, { wrap: string; dot: string }> = {
  sm: { wrap: "px-2 py-1 text-xs gap-1.5", dot: "size-1.5" },
  md: { wrap: "px-2.5 py-1.5 text-sm gap-2", dot: "size-2" },
};

function Badge({ tone, dot = true, size = "sm", className, children, ...rest }: BadgeProps) {
  const tones = toneClasses[tone];
  const sizes = sizeClasses[size];

  return (
    <span
      className={cn(
        "inline-flex items-center rounded-sm border font-semibold",
        tones.wrap,
        sizes.wrap,
        className
      )}
      {...rest}
    >
      {dot && <span className={cn("shrink-0", tones.dot, sizes.dot)} aria-hidden="true" />}
      {children}
    </span>
  );
}

export { Badge };
export type { BadgeProps, BadgeTone, BadgeSize };
