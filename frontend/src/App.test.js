import { describe, it, expect } from 'vitest';

describe('App', () => {
  it('should pass sanity check', () => {
    expect(true).toBe(true);
  });

  it('should calculate correctly', () => {
    expect(2 + 2).toBe(4);
  });
});
