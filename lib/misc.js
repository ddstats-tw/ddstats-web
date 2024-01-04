/**
 * Creates an index of objects in an array based on a given key.
 * @param {Array} array - The input array containing objects.
 * @param {string} key - The key to be used for indexing the objects in the array.
 * @returns {Object} - An object with keys corresponding to the values of the specified key in the array's objects.
 * @example
 * const array = [{ id: 1, name: 'John' }, { id: 2, name: 'Jane' }];
 * const index = createIndex(array, 'id');
 * console.log(index); // { '1': { id: 1, name: 'John' }, '2': { id: 2, name: 'Jane' } }
 */
export function createIndex(array, key) {
    return array.reduce((acc, obj) => {
        acc[obj[key]] = obj
        return acc
    }, {})
}

/**
 * Groups an array of objects by a specified key.
 * @param {Array} array - The input array containing objects.
 * @param {string} key - The key to be used for grouping the objects in the array.
 * @returns {Object} - An object with keys corresponding to the values of the specified key in the array's objects and values as arrays of grouped objects.
 * @example
 * const array = [{ category: 'A', value: 1 }, { category: 'B', value: 2 }, { category: 'A', value: 3 }];
 * const grouped = groupBy(array, 'category');
 * console.log(grouped); // { A: [ { category: 'A', value: 1 }, { category: 'A', value: 3 } ], B: [ { category: 'B', value: 2 } ] }
 */
export function groupBy(array, key) {
    return array.reduce(function (r, a) {
        r[a[key]] = r[a[key]] || []
        r[a[key]].push(a)
        return r
    }, Object.create(null))
}

/**
 * Wraps a function with a try-catch block for consistent error handling.
 * Logs the error and rethrows it when an error occurs.
 * 
 * @param {Function} fn - The function to wrap with error handling.
 * @param {LoggerFunction} logger
 * @returns {Function} - The wrapped function with error handling.
 * @example
 * const wrappedFunction = handleErrors((arg1, arg2) => {
 *   // Your function implementation here
 * });
 */
export const handleErrors = (fn, logger) => {
    return function (...args) {
        try {
            return fn(...args)
        } catch (error) {
            logger.error(error.message)
            throw error
        }
    }
}

export function splitMappers(mappers) {
    if(Array.isArray(mappers)) {
        for (const map in mappers) {
            mappers[map].Mapper = mappers[map].Mapper.split(/, | & /)
        }
    }
    else
        mappers = mappers.split(/, | & /)
    return mappers
}

export const Months = {
    "01": "January",
    "02": "February",
    "03": "March",
    "04": "April",
    "05": "May",
    "06": "June",
    "07": "July",
    "08": "August",
    "09": "September",
    "10": "October",
    "11": "November",
    "12": "December"
}
