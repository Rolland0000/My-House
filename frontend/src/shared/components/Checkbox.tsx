import type { InputHTMLAttributes, ReactNode } from "react";
import { Check } from "lucide-react";
import { cn } from "../utils/cn";

interface CheckboxProps extends Omit<InputHTMLAttributes<HTMLInputElement>, "type"> {
  label: ReactNode;
}

/** Native checkbox wrapped in its label, so a click on the text toggles it. */
function Checkbox({ label, className, ...rest }: CheckboxProps) {
  return (
    <label
      className={cn(
        "flex cursor-pointer items-center gap-2.5 text-sm text-text has-disabled:cursor-not-allowed has-disabled:opacity-50",
        className
      )}
    >
      <span className="relative flex size-4 flex-none">
        <input
          type="checkbox"
          className={cn(
            "peer size-4 cursor-[inherit] appearance-none rounded-none border border-border-strong bg-surface",
            "transition-colors duration-120 ease-out checked:border-ink-900 checked:bg-ink-900",
            "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-focus focus-visible:ring-offset-2"
          )}
          {...rest}
        />
        <Check
          aria-hidden="true"
          strokeWidth={3}
          className="pointer-events-none absolute inset-0.5 hidden size-3 text-white peer-checked:block"
        />
      </span>
      {label}
    </label>
  );
}

export { Checkbox };
export type { CheckboxProps };
