import { createBrowserRouter } from "react-router";
import { RootLayout } from "./layout/RootLayout";

// eslint-disable-next-line react-refresh/only-export-components -- placeholder page colocated until a real home feature exists
function HomePage() {
  return <h1>MyHouse</h1>;
}

export const router = createBrowserRouter([
  {
    path: "/",
    Component: RootLayout,
    children: [{ index: true, Component: HomePage }],
  },
]);
