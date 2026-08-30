import { useEffect, useState } from "react";

export function useCountdown() {
  const [secondsLeft, setSecondsLeft] = useState(0);

  useEffect(() => {
    if (secondsLeft <= 0) return;
    const timeout = window.setTimeout(() => setSecondsLeft((current) => current - 1), 1000);
    return () => window.clearTimeout(timeout);
  }, [secondsLeft]);

  return { secondsLeft, start: setSecondsLeft };
}
