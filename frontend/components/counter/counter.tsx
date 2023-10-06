import { CounterProps } from "./props";
const Counter = ({
  value,
  fetchNext,
  fetchPrevious
}: CounterProps) => {
  return (
    <div className="flex flex-col gap-8">
      <span className="text-3xl text-lime-400 font-bold">{value}</span>
      <div className="flex flex-row gap-4">
        <button
          className="h-12 bg-lime-400 w-36 text-black rounded-md text-2xl font-bold"
          onClick={
            fetchPrevious
          }>Previous</button>
        <button
          className="h-12 bg-lime-400 w-36 text-black rounded-md text-2xl font-bold"
          onClick={
            fetchNext
          }>Next</button>
      </div>
    </div>
  );
}

export default Counter;