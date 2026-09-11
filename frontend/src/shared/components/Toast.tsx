import { createContext, useCallback, useContext, useMemo, useState, type ReactNode } from "react";
import { createPortal } from "react-dom";
import { CheckCircle2, Info, TriangleAlert, CircleX, X } from "lucide-react";
import { cn } from "../utils/cn";

type ToastVariant = "success" | "warning" | "error" | "info";

interface ToastItem {
  id: string;
  message: string;
  variant: ToastVariant;
}

interface ToastOptions {
  variant?: ToastVariant;
  durationMs?: number;
}

interface ToastContextValue {
  showToast: (message: string, options?: ToastOptions) => void;
}

const ToastContext = createContext<ToastContextValue | null>(null);

const variantConfig: Record<
  ToastVariant,
  { icon: typeof Info; classes: string; iconClasses: string; persistent: boolean }
> = {
  success: {
    icon: CheckCircle2,
    classes: "border-success/30",
    iconClasses: "text-success",
    persistent: false,
  },
  warning: {
    icon: TriangleAlert,
    classes: "border-warning/35",
    iconClasses: "text-warning-text",
    persistent: true,
  },
  error: {
    icon: CircleX,
    classes: "border-error/35",
    iconClasses: "text-error-text",
    persistent: true,
  },
  info: {
    icon: Info,
    classes: "border-ink-600/30",
    iconClasses: "text-ink-600",
    persistent: false,
  },
};

const DEFAULT_DURATION_MS = 4000;

function ToastProvider({ children }: { children: ReactNode }) {
  const [toasts, setToasts] = useState<ToastItem[]>([]);

  const dismissToast = useCallback((id: string) => {
    setToasts((current) => current.filter((toast) => toast.id !== id));
  }, []);

  const showToast = useCallback(
    (message: string, options?: ToastOptions) => {
      const id = crypto.randomUUID();
      const variant = options?.variant ?? "info";

      setToasts((current) => [...current, { id, message, variant }]);

      // Warning/error stay until manually closed unless a duration is forced explicitly.
      const durationMs =
        options?.durationMs ?? (variantConfig[variant].persistent ? null : DEFAULT_DURATION_MS);
      if (durationMs !== null) {
        window.setTimeout(() => dismissToast(id), durationMs);
      }
    },
    [dismissToast]
  );

  const value = useMemo(() => ({ showToast }), [showToast]);

  return (
    <ToastContext.Provider value={value}>
      {children}
      {createPortal(
        <div
          className="pointer-events-none fixed inset-x-0 bottom-4 z-50 flex flex-col items-center gap-2 sm:inset-x-auto sm:left-4 sm:items-start"
          aria-live="polite"
        >
          {toasts.map((toast) => {
            const { icon: Icon, classes, iconClasses } = variantConfig[toast.variant];
            return (
              <div
                key={toast.id}
                role="status"
                className={cn(
                  "pointer-events-auto flex w-80 max-w-[calc(100vw-2rem)] items-start gap-3 rounded-lg border bg-surface px-4 py-3 text-sm shadow-elevated",
                  classes
                )}
              >
                <Icon className={cn("size-5 shrink-0", iconClasses)} aria-hidden="true" />
                <p className="flex-1 text-text">{toast.message}</p>
                <button
                  type="button"
                  aria-label="Close notification"
                  onClick={() => dismissToast(toast.id)}
                  className="text-text-muted hover:text-text focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-focus"
                >
                  <X className="size-4" aria-hidden="true" />
                </button>
              </div>
            );
          })}
        </div>,
        document.body
      )}
    </ToastContext.Provider>
  );
}

function useToast(): ToastContextValue {
  const context = useContext(ToastContext);
  if (!context) {
    throw new Error("useToast must be used within a ToastProvider");
  }
  return context;
}

export { ToastProvider, useToast };
export type { ToastVariant, ToastOptions };
