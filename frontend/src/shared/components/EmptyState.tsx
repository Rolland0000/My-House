import { Button } from "./Button";
import { cn } from "../utils/cn";

interface EmptyStateAction {
  label: string;
  onClick: () => void;
}

interface EmptyStateProps {
  title: string;
  description?: string;
  primaryAction?: EmptyStateAction;
  secondaryAction?: EmptyStateAction;
  className?: string;
}

function EmptyState({
  title,
  description,
  primaryAction,
  secondaryAction,
  className,
}: EmptyStateProps) {
  return (
    <div
      className={cn(
        "flex flex-col items-center gap-1 rounded-sm border border-border bg-surface px-10 py-16 text-center",
        className
      )}
    >
      <div
        aria-hidden="true"
        className="mb-5 size-11 border border-border-strong"
        style={{
          backgroundImage: "repeating-linear-gradient(135deg, #f1f2ee 0 5px, #ffffff 5px 10px)",
        }}
      />
      <h2 className="text-lg font-bold text-text">{title}</h2>
      {description && <p className="mx-auto max-w-[46ch] text-sm text-text-muted">{description}</p>}
      {(primaryAction || secondaryAction) && (
        <div className="mt-4 flex flex-wrap justify-center gap-3">
          {primaryAction && <Button onClick={primaryAction.onClick}>{primaryAction.label}</Button>}
          {secondaryAction && (
            <Button variant="secondary" onClick={secondaryAction.onClick}>
              {secondaryAction.label}
            </Button>
          )}
        </div>
      )}
    </div>
  );
}

export { EmptyState };
export type { EmptyStateProps };
