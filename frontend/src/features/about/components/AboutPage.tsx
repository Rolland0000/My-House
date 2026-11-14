import { Link } from "react-router";
import { MessageSquareText, Search, ShieldCheck } from "lucide-react";
import { Card, buttonClasses } from "../../../shared/components";
import { cn } from "../../../shared/utils/cn";

const STEPS = [
  {
    icon: Search,
    title: "Search",
    text: "Browse listings or filter by city, type and price to find a place that fits.",
  },
  {
    icon: MessageSquareText,
    title: "Contact the owner",
    text: "Sign in and reveal the owner's phone numbers to arrange a visit directly.",
  },
  {
    icon: ShieldCheck,
    title: "Visit, then agree",
    text: "See the property first. The agreement is between you and the owner.",
  },
] as const;

function AboutPage() {
  return (
    <div className="mx-auto w-full max-w-[1112px] px-4 py-10 sm:px-7">
      <header className="mb-10 max-w-2xl">
        <h1 className="text-3xl font-bold text-ink-900">About My House</h1>
        <p className="mt-3 text-lg text-text">
          My House connects people looking for a home to rent with the owners who have one to offer,
          in Cameroon and French-speaking Africa.
        </p>
      </header>

      <section aria-labelledby="mission" className="mb-10 max-w-2xl">
        <h2 id="mission" className="mb-2 text-xl font-semibold text-ink-900">
          Why we exist
        </h2>
        <p className="text-text">
          Finding a rental often depends on word of mouth and unreliable ads. We give owners a
          simple place to publish their properties, and renters a clear way to compare them and
          reach the owner directly, without a middleman.
        </p>
      </section>

      <section aria-labelledby="how-it-works" className="mb-10">
        <h2 id="how-it-works" className="mb-4 text-xl font-semibold text-ink-900">
          How it works
        </h2>
        <ol className="grid gap-4 md:grid-cols-3">
          {STEPS.map(({ icon: Icon, title, text }, index) => (
            <li key={title}>
              <Card className="h-full">
                <div className="mb-3 flex items-center gap-3">
                  <span className="flex size-8 items-center justify-center bg-ink-900 text-sm font-semibold text-white">
                    {index + 1}
                  </span>
                  <Icon className="size-5 text-ink-500" aria-hidden="true" />
                </div>
                <h3 className="font-semibold text-ink-900">{title}</h3>
                <p className="mt-1 text-sm text-text-muted">{text}</p>
              </Card>
            </li>
          ))}
        </ol>
      </section>

      <section aria-labelledby="owners" className="mb-10 max-w-2xl">
        <h2 id="owners" className="mb-2 text-xl font-semibold text-ink-900">
          For owners
        </h2>
        <p className="text-text">
          Owners are verified by our team before they can publish. Once approved, you manage your
          properties, photos and availability yourself.
        </p>
      </section>

      <section aria-labelledby="safety" className="mb-10 max-w-2xl">
        <h2 id="safety" className="mb-2 text-xl font-semibold text-ink-900">
          Staying safe
        </h2>
        <p className="text-text">
          Never send money before visiting the property. My House does not hold deposits and is not
          a party to the agreement between renter and owner.
        </p>
      </section>

      <div className="flex flex-wrap gap-3">
        <Link to="/search" className={cn(buttonClasses())}>
          Browse listings
        </Link>
        <Link to="/contact" className={cn(buttonClasses("secondary"))}>
          Contact us
        </Link>
      </div>
    </div>
  );
}

export { AboutPage };
