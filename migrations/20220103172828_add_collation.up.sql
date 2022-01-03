-- Ignores case
CREATE COLLATION ignore_case (
  provider = icu,
  locale = 'und-u-ks-level1',
  deterministic = false
);
