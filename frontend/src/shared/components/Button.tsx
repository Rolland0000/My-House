import type { ButtonHTMLAttributes, ReactNode } from "react";
import { cn } from "../utils/cn";
import { Spinner } from "./Spinner";

type ButtonVariant = "primary" | "secondary" | "ghost" | "danger" | "danger-outline";
type ButtonSize = "sm" | "md";

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: ButtonVariant;
  size?: ButtonSize;
  isLoading?: boolean;
  leftIcon?: ReactNode;
  rightIcon?: ReactNode;
}

const baseClasses =
  "inline-flex items-center justify-center gap-2 rounded-sm font-semibold transition-colors duration-120 ease-out " +
  "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-focus focus-visible:ring-offset-2 " +
  "disabled:cursor-not-allowed disabled:opacity-50";

const variantClasses: Record<ButtonVariant, string> = {
  primary:
    "border border-brass-600 bg-primary text-ink-900 hover:border-brass-600 hover:bg-brass-600",
  secondary: "border border-ink-600 bg-surface text-ink-600 hover:bg-primary-soft",
  ghost: "bg-transparent text-ink-600 hover:bg-primary-soft",
  danger: "border border-error bg-error text-white hover:opacity-90",
  "danger-outline": "border border-error bg-surface text-error hover:bg-error-soft",
};

const sizeClasses: Record<ButtonSize, string> = {
  sm: "px-3 py-1.5 text-sm",
  md: "px-5 py-3 text-base",
};

function Button({
  variant = "primary",
  size = "md",
  isLoading = false,
  leftIcon,
  rightIcon,
  disabled,
  children,
  className,
  ...rest
}: ButtonProps) {
  return (
    <button
      className={cn(baseClasses, variantClasses[variant], sizeClasses[size], className)}
      disabled={disabled || isLoading}
      {...rest}
    >
      {isLoading ? <Spinner size="sm" /> : leftIcon}
      {children}
      {!isLoading && rightIcon}
    </button>
  );
}

export { Button };
export type { ButtonProps, ButtonVariant, ButtonSize };
