import type { InputHTMLAttributes, ReactNode } from "react";
import { cn } from "../utils/cn";

interface InputProps extends InputHTMLAttributes<HTMLInputElement> {
  hasError?: boolean;
  leftIcon?: ReactNode;
  rightIcon?: ReactNode;
}

const baseClasses =
  "w-full rounded-sm border bg-surface px-3 py-2 text-base text-text placeholder:text-text-muted " +
  "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-focus " +
  "disabled:cursor-not-allowed disabled:opacity-50";

function Input({ hasError = false, leftIcon, rightIcon, className, ...rest }: InputProps) {
  if (!leftIcon && !rightIcon) {
    return (
      <input
        className={cn(baseClasses, hasError ? "border-error" : "border-border", className)}
        {...rest}
      />
    );
  }

  return (
    <div className="relative flex items-center">
      {leftIcon && (
        <span className="pointer-events-none absolute left-3 flex items-center text-text-muted">
          {leftIcon}
        </span>
      )}
      <input
        className={cn(
          baseClasses,
          hasError ? "border-error" : "border-border",
          leftIcon && "pl-9",
          rightIcon && "pr-9",
          className
        )}
        {...rest}
      />
      {rightIcon && (
        <span className="absolute right-3 flex items-center text-text-muted">{rightIcon}</span>
      )}
    </div>
  );
}

export { Input };
export type { InputProps };
