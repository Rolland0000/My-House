import type { HTMLAttributes } from "react";
import { cn } from "../utils/cn";

type CardPadding = "none" | "sm" | "md";
type CardRadius = "none" | "sm";

interface CardProps extends HTMLAttributes<HTMLDivElement> {
  padding?: CardPadding;
  radius?: CardRadius;
}

const paddingClasses: Record<CardPadding, string> = {
  none: "",
  sm: "p-4",
  md: "p-6",
};

const radiusClasses: Record<CardRadius, string> = {
  none: "rounded-none",
  sm: "rounded-sm",
};

function Card({ padding = "md", radius = "sm", className, children, ...rest }: CardProps) {
  return (
    <div
      className={cn(
        "border border-border bg-surface",
        radiusClasses[radius],
        paddingClasses[padding],
        className
      )}
      {...rest}
    >
      {children}
    </div>
  );
}

export { Card };
export type { CardProps, CardRadius };
