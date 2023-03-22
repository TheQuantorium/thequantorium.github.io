export function formatDate(year, month, day) {
  const opts = { dateStyle: "short" };
  const date = new Date(year, month, day);

  const dateTimeFormat = new Intl.DateTimeFormat(undefined, opts);
  return dateTimeFormat.format(date);
}
