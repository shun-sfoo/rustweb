import { useEffect, useState } from 'react';

export const isVoid = (value: unknown) =>
  value === undefined || value === null || value === '';

export const cleanObject = (object: { [key: string]: unknown }) => {
  const result = { ...object };

  Object.keys(result).forEach((key) => {
    const value = result[key];
    if (isVoid(value)) {
      delete result[key];
    }
  });
  return result;
};

export const useMount = (callback: () => void) => {
  useEffect(() => {
    callback();
  }, []);
};

// debounce 用于将一些快速事件合并成一次
// const debounce = (func, delay) => {
//   let timeout;
//   return (...param) => {
//     if (timeout) {
//       clearTimeout(timeout); //删除计时事件
//     }
//
//     timeout = setTimeout(function () {
//       func(...param);
//     }, delay);
//   };
// };
//
// const log = debounce(() => console.log('call'), 5000);
// log();
// log();
// log();
// debounce 原理:
// 0s ------------> 1s -------------> 2s -----------> ...
//    一定要理解： 这三个函数都是同步操作，所以他们都是在0~1s这个时间段内瞬间完成的:
//    log()#1 // timeout#1
//    log()#2 // 发现 timeout#1! 取消之，然后设置timeout#2
//    log()#3 // 发现 timeout#2! 取消之，然后设置timeout#3
//         // 所以 log() #3 结束后，就只剩下 timeout#3在独自等待了
//

export const useDebounce = <V,>(value: V, delay?: number) => {
  const [debouncedValue, setDebouncedValue] = useState(value);

  useEffect(() => {
    // 每次在value (_delay)变化以后，设置一个定时器
    const timeout = setTimeout(() => setDebouncedValue(value), delay);
    // 每次在上一次useEffect处理完成以后再运行
    return () => clearTimeout(timeout);
  });

  return debouncedValue;
};

export const useArray = <T,>(initialArray: T[]) => {
  const [value, setValue] = useState(initialArray);

  return {
    value,
    setValue,
    add: (item: T) => setValue([...value, item]),
    clear: () => setValue([]),
    removeIdex: (index: number) => {
      const copy = [...value];
      copy.splice(index, 1);
      setValue(copy);
    },
  };
};
