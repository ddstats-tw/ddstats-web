// Auto complete
let cacheSearch = {}
let currentValue

let search_ids = ["search-home", "search-nav"]
for (let id of search_ids) {
    const input = document.getElementById(id)
    if(input) {
        input.addEventListener("input", onInput)
        input.addEventListener("keydown", onKeydown)
    }
}

function onInput(event) {
    const value = encodeURIComponent(event.target.value)
    const id = event.target.id.split("-")[1]

    if(!value.length) {
        const resultDiv = document.getElementById(`result-${id}`)
        if(resultDiv)
            resultDiv.innerHTML = ""
        return
    }
    currentValue = value
    if (cacheSearch[value])
        showResults(value, cacheSearch[value], id)
    else {
        fetch("/search/api?id=" + id + "&" + "q=" + value)
            .then(response => {
                if(response.ok)
                    return response.text()
                
                throw new Error("Failed to get autocompletions")
            })
            .then(data => {
                cacheSearch[value] = data
                showResults(value, data, id)
            })
    }
}

function showResults(value, results, id) {
    if(value != currentValue)
        return

    const resultDiv = document.getElementById(`result-${id}`)
    if(!resultDiv)
        return

    resultDiv.innerHTML = results
}

function onKeydown(event) {
    const id = event.target.id.split("-")[1]
    let resultList = document.getElementById(`result-list-${id}`)

    if(!resultList)
        return

    const selectedItemID = `selected-${id}`
    const selectedItem = resultList.querySelector("." + selectedItemID)

    if(event.key === "ArrowDown") {
        if(selectedItem && selectedItem.nextElementSibling) {
            selectedItem.nextElementSibling.classList.add(selectedItemID)
            selectedItem.classList.remove(selectedItemID)
        } else {
            resultList.firstElementChild.classList.add(selectedItemID)
            resultList.lastElementChild.classList.remove(selectedItemID)
        }
    } else if (event.key === "ArrowUp") {
        if(selectedItem && selectedItem.previousElementSibling) {
            selectedItem.previousElementSibling.classList.add(selectedItemID)
            selectedItem.classList.remove(selectedItemID)
        } else {
            resultList.lastElementChild.classList.add(selectedItemID)
            resultList.firstElementChild.classList.remove(selectedItemID)
        }
    } else if (event.key === "Enter") {
        if(resultList.firstElementChild && resultList.firstElementChild == resultList.lastElementChild) {
            event.preventDefault()
            window.location.href = resultList.firstElementChild.firstElementChild.getAttribute("href")
        }
        if(selectedItem) {
            event.preventDefault()
            window.location.href = selectedItem.firstElementChild.getAttribute("href")
        }
    }
}
