import { cloneElement, isValidElement, useId, type ReactElement } from "react";
import { cn } from "../utils/cn";

interface FormFieldChildProps {
  id?: string;
  "aria-describedby"?: string;
  "aria-invalid"?: boolean;
}

interface FormFieldProps {
  label: string;
  htmlFor?: string;
  hint?: string;
  error?: string;
  required?: boolean;
  className?: string;
  children: ReactElement<FormFieldChildProps>;
}

function FormField({
  label,
  htmlFor,
  hint,
  error,
  required = false,
  className,
  children,
}: FormFieldProps) {
  const generatedId = useId();
  const fieldId = htmlFor ?? generatedId;
  const hintId = hint ? `${fieldId}-hint` : undefined;
  const errorId = error ? `${fieldId}-error` : undefined;
  const describedBy = [hintId, errorId].filter(Boolean).join(" ") || undefined;

  const child = isValidElement(children)
    ? cloneElement(children, {
        id: fieldId,
        "aria-describedby": describedBy,
        "aria-invalid": Boolean(error) || undefined,
      })
    : children;

  return (
    <div className={cn("flex flex-col gap-1.5", className)}>
      <label htmlFor={fieldId} className="text-sm font-semibold text-text">
        {label}
        {required && (
          <span className="ml-0.5 text-error" aria-hidden="true">
            *
          </span>
        )}
      </label>

      {child}

      {hint && !error && (
        <p id={hintId} className="text-sm text-text-muted">
          {hint}
        </p>
      )}

      {error && (
        <p id={errorId} role="alert" className="text-sm text-error">
          {error}
        </p>
      )}
    </div>
  );
}

export { FormField };
export type { FormFieldProps };
