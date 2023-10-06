import Image from 'next/image'
import { Inter } from 'next/font/google'
import Counter from '@/components/counter/counter'
import { useDispatch, useSelector } from 'react-redux';
import { useEffect } from 'react';
import { fetchInitialState, fetchNextCounter, fetchPreviousCounter } from '@/redux/fib/reducer';


export default function Home() {
  const dispatch = useDispatch();
  const fib = useSelector((state: any) => state.fib.value);

  useEffect(() => {
    dispatch(fetchInitialState() as any);
  }
    , [dispatch])


  const handlePrevious = () => {
    dispatch(fetchPreviousCounter() as any);
  };

  const handleNext = () => {
    dispatch(fetchNextCounter() as any);
  };



  return (
    <main className="flex flex-col items-center justify-center w-full flex-1 px-20 text-center">
      <Counter value={fib} fetchNext={handleNext} fetchPrevious={handlePrevious} />
    </main>
  )
}
