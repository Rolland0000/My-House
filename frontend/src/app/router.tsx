import { lazy, Suspense, type ComponentType } from "react";
import { createBrowserRouter } from "react-router";
import { RootLayout } from "./layout/RootLayout";
import { Spinner } from "../shared/components";

const ListingFeed = lazy(() =>
  import("../features/listings/components/ListingFeed").then((m) => ({ default: m.ListingFeed }))
);
const ListingDetail = lazy(() =>
  import("../features/listings/components/ListingDetail").then((m) => ({
    default: m.ListingDetail,
  }))
);

function withSuspense(Component: ComponentType) {
  return (
    <Suspense
      fallback={
        <div className="flex justify-center py-16">
          <Spinner size="lg" label="Chargement…" />
        </div>
      }
    >
      <Component />
    </Suspense>
  );
}

export const router = createBrowserRouter([
  {
    path: "/",
    Component: RootLayout,
    children: [
      { index: true, element: withSuspense(ListingFeed) },
      { path: "listings/:id", element: withSuspense(ListingDetail) },
    ],
  },
]);
