import { cn } from "../utils/cn";

type SkeletonVariant = "card" | "line" | "block";

interface SkeletonProps {
  variant: SkeletonVariant;
  aspectRatio?: string;
  className?: string;
}

/**
 * Fixed #EDEEEA / #F1F2EE fill, no shimmer — holds the perf budget and respects
 * `prefers-reduced-motion` by default since nothing animates.
 */
function Skeleton({ variant, aspectRatio = "4/3", className }: SkeletonProps) {
  if (variant === "line") {
    return <div className={cn("h-3.5 rounded-sm bg-[#EDEEEA]", className)} />;
  }

  if (variant === "block") {
    return <div className={cn("rounded-sm bg-[#EDEEEA]", className)} />;
  }

  return (
    <div className={cn("border border-border bg-surface", className)}>
      <div className="bg-[#EDEEEA]" style={{ aspectRatio }} />
      <div className="flex flex-col gap-2 p-4">
        <div className="h-3.5 w-3/4 bg-[#EDEEEA]" />
        <div className="h-2.5 w-1/3 bg-[#F1F2EE]" />
      </div>
    </div>
  );
}

export { Skeleton };
export type { SkeletonProps, SkeletonVariant };
