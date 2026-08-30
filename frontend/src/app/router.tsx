import { lazy, Suspense, type ComponentType } from "react";
import { createBrowserRouter } from "react-router";
import { AuthLayout } from "./layout/AuthLayout";
import { RequireAuth } from "./RequireAuth";
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
const AuthFlow = lazy(() =>
  import("../features/auth/components/AuthFlow").then((m) => ({ default: m.AuthFlow }))
);
const ProfileForm = lazy(() =>
  import("../features/profile/components/ProfileForm").then((m) => ({ default: m.ProfileForm }))
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
      {
        path: "profile",
        element: <RequireAuth>{withSuspense(ProfileForm)}</RequireAuth>,
      },
    ],
  },
  {
    path: "/login",
    Component: AuthLayout,
    children: [{ index: true, element: withSuspense(AuthFlow) }],
  },
]);
