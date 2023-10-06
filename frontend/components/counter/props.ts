export interface CounterProps {
  value: number;
  fetchNext: () => void;
  fetchPrevious: () => void;
}
