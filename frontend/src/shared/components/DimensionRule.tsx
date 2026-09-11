import { cn } from "../utils/cn";

interface DimensionRuleProps {
  width?: number | "full";
  className?: string;
}

const DEFAULT_WIDTH = 120;

/**
 * The signature "dimension rule" device: a rule-400 line with two perpendicular
 * end-ticks, like the dimension line on an architect's drawing. Exactly two
 * permitted uses — a short lead-in under a title (default 120px), or a full-width
 * section separator (`width="full"`). Never a third treatment.
 */
function DimensionRule({ width = DEFAULT_WIDTH, className }: DimensionRuleProps) {
  const style = width === "full" ? undefined : { width };

  return (
    <div
      role="separator"
      aria-hidden="true"
      className={cn("flex h-[9px] items-center", width === "full" && "w-full", className)}
      style={style}
    >
      <span className="h-[9px] w-px shrink-0 bg-border-strong" />
      <span className="h-px flex-1 bg-border-strong" />
      <span className="h-[9px] w-px shrink-0 bg-border-strong" />
    </div>
  );
}

export { DimensionRule };
export type { DimensionRuleProps };
