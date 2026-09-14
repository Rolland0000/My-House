import { lazy, Suspense, type ComponentType } from "react";
import { createBrowserRouter, Navigate } from "react-router";
import { AuthLayout } from "./layout/AuthLayout";
import { RequireAuth } from "./RequireAuth";
import { RequireAdmin } from "./RequireAdmin";
import { RootLayout } from "./layout/RootLayout";
import { AdminLayout } from "./layout/AdminLayout";
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
const OwnerRequestForm = lazy(() =>
  import("../features/owner-request/components/OwnerRequestForm").then((m) => ({
    default: m.OwnerRequestForm,
  }))
);
const OwnerRequestStatus = lazy(() =>
  import("../features/owner-request/components/OwnerRequestStatus").then((m) => ({
    default: m.OwnerRequestStatus,
  }))
);
const OwnerRequestQueueList = lazy(() =>
  import("../features/admin/components/OwnerRequestQueueList").then((m) => ({
    default: m.OwnerRequestQueueList,
  }))
);
const OwnerRequestDetail = lazy(() =>
  import("../features/admin/components/OwnerRequestDetail").then((m) => ({
    default: m.OwnerRequestDetail,
  }))
);

function withSuspense(Component: ComponentType) {
  return (
    <Suspense
      fallback={
        <div className="flex justify-center py-16">
          <Spinner size="lg" label="Loading…" />
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
      {
        path: "owner-request",
        element: <RequireAuth>{withSuspense(OwnerRequestForm)}</RequireAuth>,
      },
      {
        path: "owner-request/status",
        element: <RequireAuth>{withSuspense(OwnerRequestStatus)}</RequireAuth>,
      },
    ],
  },
  {
    path: "/login",
    Component: AuthLayout,
    children: [{ index: true, element: withSuspense(AuthFlow) }],
  },
  {
    path: "/admin",
    element: (
      <RequireAuth>
        <RequireAdmin>
          <AdminLayout />
        </RequireAdmin>
      </RequireAuth>
    ),
    children: [
      { index: true, element: <Navigate to="owner-requests" replace /> },
      { path: "owner-requests", element: withSuspense(OwnerRequestQueueList) },
      { path: "owner-requests/:id", element: withSuspense(OwnerRequestDetail) },
    ],
  },
]);
