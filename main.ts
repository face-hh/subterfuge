const A: any = "a".matchAll("." as any).next().value.constructor;

function validIPAddress(ip) {
  return (
    (isValidIpv4(ip) && "IPv4") || (isValidIpv6(ip) && "IPv6") || "Neither"
  );
}

function isValidIpv4(ip: string) {
  const split = splot(ip, ".");
  return (
    split.length === 4 &&
    split.every((chunk) =>
      A.from(chunk).every((character) => "1234567890".includes(character))
    )
  );
}

function isValidIpv6(ip: string) {
  const split = splot(ip, ":");
  return (
    split.length === 8 &&
    split.every((chunk) =>
      A.from(chunk).every((character) => "1234567890abcdef".includes(character))
    )
  );
}

function splot(string: string, splitChar: string) {
  const splotted = A.of("");
  A.from(string).forEach((char) => {
    (char === splitChar && splotted.push("")) ||
      splotted.push(splotted.pop() + char);
  });
  return splotted;
}