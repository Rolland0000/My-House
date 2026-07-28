import type { HTMLAttributes } from "react";
import { cn } from "../utils/cn";

type CardPadding = "none" | "sm" | "md";

interface CardProps extends HTMLAttributes<HTMLDivElement> {
  padding?: CardPadding;
}

const paddingClasses: Record<CardPadding, string> = {
  none: "",
  sm: "p-4",
  md: "p-6",
};

function Card({ padding = "md", className, children, ...rest }: CardProps) {
  return (
    <div
      className={cn(
        "rounded-md border border-border bg-surface",
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
export type { CardProps };
