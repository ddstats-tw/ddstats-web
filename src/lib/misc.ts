import type { Logger } from "./logger"

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
export function createIndex(array: any[], key: string): object {
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
export function groupBy(array: any[], key: string) {
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
 * @param {Logger} logger
 * @returns {Function} - The wrapped function with error handling.
 * @examples
 * const wrappedFunction = handleErrors((arg1, arg2) => {
 *   // Your function implementation here
 * });
 */
export const handleErrors = (fn: Function, logger: Logger): Function => {
    return function (...args: any) {
        try {
            return fn(...args)
        } catch (error: unknown) {
            if(error instanceof Error)
            {
                logger.error(error.message)
                throw error
            }
        }
    }
}

export function splitMappers(mappers: string) {
    return mappers.split(/, | & /)
}

export function escapeFTS(query: string): string {
    // If query has unbalanced ", add one at end
    if ((query.match(/"/g) || []).length % 2 !== 0) {
        query += "\""
    }

    // Split the query based on whitespace or quoted strings
    let bits = query.split(/\s+|(".*?")/).filter(Boolean)

    // Remove empty strings and double quotes
    bits = bits.filter((b: string) => b && b !== "\"\"")

    // Rejoin the bits with quotes, unless they already start with a quote
    return bits.map((bit: string) => bit.startsWith("\"") ? bit : `"*${bit}*"`).join(" ")
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
