import { formatMonthlyRent } from "../../../shared/utils/format";

interface ListingPriceProps {
  price: number;
  className?: string;
  unitClassName?: string;
}

function ListingPrice({ price, className, unitClassName }: ListingPriceProps) {
  const { amount, unit } = formatMonthlyRent(price);

  return (
    <span className={className}>
      {amount} <span className={unitClassName}>{unit}</span>
    </span>
  );
}

export { ListingPrice };
