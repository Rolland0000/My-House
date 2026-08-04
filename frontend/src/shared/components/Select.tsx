import type { SelectHTMLAttributes } from "react";
import { ChevronDown } from "lucide-react";
import { cn } from "../utils/cn";

interface SelectOption {
  value: string;
  label: string;
  disabled?: boolean;
}

interface SelectProps extends SelectHTMLAttributes<HTMLSelectElement> {
  hasError?: boolean;
  options: SelectOption[];
  placeholder?: string;
}

function Select({ hasError = false, options, placeholder, className, ...rest }: SelectProps) {
  return (
    <div className="relative flex items-center">
      <select
        className={cn(
          "w-full appearance-none rounded-sm border bg-surface px-3 py-2 pr-9 text-base text-text " +
            "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-focus " +
            "disabled:cursor-not-allowed disabled:opacity-50",
          hasError ? "border-error" : "border-border",
          className
        )}
        {...rest}
      >
        {placeholder && (
          <option value="" disabled hidden>
            {placeholder}
          </option>
        )}
        {options.map((option) => (
          <option key={option.value} value={option.value} disabled={option.disabled}>
            {option.label}
          </option>
        ))}
      </select>
      <ChevronDown
        className="pointer-events-none absolute right-3 size-4 text-text-muted"
        aria-hidden="true"
      />
    </div>
  );
}

export { Select };
export type { SelectProps, SelectOption };
