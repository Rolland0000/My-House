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

const variantConfig: Record<ToastVariant, { icon: typeof Info; classes: string }> = {
  success: { icon: CheckCircle2, classes: "border-success bg-success-soft text-success" },
  warning: { icon: TriangleAlert, classes: "border-warning bg-warning-soft text-warning" },
  error: { icon: CircleX, classes: "border-error bg-error-soft text-error" },
  info: { icon: Info, classes: "border-focus bg-primary-soft text-text" },
};

const DEFAULT_DURATION_MS = 5000;

function ToastProvider({ children }: { children: ReactNode }) {
  const [toasts, setToasts] = useState<ToastItem[]>([]);

  const dismissToast = useCallback((id: string) => {
    setToasts((current) => current.filter((toast) => toast.id !== id));
  }, []);

  const showToast = useCallback(
    (message: string, options?: ToastOptions) => {
      const id = crypto.randomUUID();
      const variant = options?.variant ?? "info";
      const durationMs = options?.durationMs ?? DEFAULT_DURATION_MS;

      setToasts((current) => [...current, { id, message, variant }]);
      window.setTimeout(() => dismissToast(id), durationMs);
    },
    [dismissToast]
  );

  const value = useMemo(() => ({ showToast }), [showToast]);

  return (
    <ToastContext.Provider value={value}>
      {children}
      {createPortal(
        <div
          className="pointer-events-none fixed bottom-4 right-4 z-50 flex flex-col gap-2"
          aria-live="polite"
        >
          {toasts.map((toast) => {
            const { icon: Icon, classes } = variantConfig[toast.variant];
            return (
              <div
                key={toast.id}
                role="status"
                className={cn(
                  "pointer-events-auto flex w-80 items-start gap-3 rounded-md border px-4 py-3 text-sm shadow-lg",
                  classes
                )}
              >
                <Icon className="size-5 shrink-0" aria-hidden="true" />
                <p className="flex-1 text-text">{toast.message}</p>
                <button
                  type="button"
                  aria-label="Fermer la notification"
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
