import { describe, expect, it } from "vitest";
import {
  MAX_SEARCH_CHARS,
  parseAdminUsersCriteria,
  toAdminUsersRequest,
  toAdminUsersUrlParams,
} from "./adminUsersCriteria";

const parse = (query: string) => parseAdminUsersCriteria(new URLSearchParams(query));

describe("parseAdminUsersCriteria", () => {
  it("reads every criterion", () => {
    expect(parse("q=%20ada%20&role=owner&state=suspended&page=3")).toEqual({
      q: "ada",
      role: "owner",
      state: "suspended",
      page: 3,
    });
  });

  it("treats invalid values as absent", () => {
    expect(parse("role=root&state=inactive&page=abc")).toEqual({
      q: "",
      role: "",
      state: "",
      page: 1,
    });
  });

  it("drops a search text over the limit, counted in characters", () => {
    expect(parse(`q=${"é".repeat(MAX_SEARCH_CHARS)}`).q).toHaveLength(MAX_SEARCH_CHARS);
    expect(parse(`q=${"a".repeat(MAX_SEARCH_CHARS + 1)}`).q).toBe("");
  });
});

describe("toAdminUsersUrlParams", () => {
  it("round-trips through the parser", () => {
    const criteria = { q: "ada", role: "seeker", state: "active", page: 2 } as const;
    expect(parseAdminUsersCriteria(toAdminUsersUrlParams(criteria))).toEqual(criteria);
  });

  it("leaves out empty values and page 1", () => {
    const params = toAdminUsersUrlParams({ q: "", role: "", state: "", page: 1 });
    expect(params.toString()).toBe("");
  });
});

describe("toAdminUsersRequest", () => {
  it("sends no empty filter to the API", () => {
    expect(toAdminUsersRequest({ q: "", role: "", state: "", page: 2 })).toEqual({
      q: undefined,
      role: undefined,
      state: undefined,
      page: 2,
    });
  });
});
