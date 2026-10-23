import { describe, expect, it } from "vitest";
import { greetingForHour } from "./greeting";

describe("greetingForHour", () => {
  it.each([
    [0, "晚上好"],
    [4, "晚上好"],
    [5, "早上好"],
    [10, "早上好"],
    [11, "中午好"],
    [13, "中午好"],
    [14, "下午好"],
    [17, "下午好"],
    [18, "晚上好"],
    [23, "晚上好"],
  ])("hour %i -> %s", (hour, expected) => {
    expect(greetingForHour(hour)).toBe(expected);
  });
});
