// Auto complete
let cacheSearch = {}

let search_ids = ["search-home", "search-nav"]
for (let id of search_ids) {
    const input = document.getElementById(id)
    if(input) {
        input.addEventListener("input", onInput)
        input.addEventListener("keydown", onKeydown)
    }
}

function onInput(event) {
    const value = event.target.value
    const id = event.target.id.split("-")[1]

    if(!value.length) {
        const resultDiv = document.getElementById(`result-${id}`)
        if(resultDiv)
            resultDiv.innerHTML = ""
        return
    }
    if (cacheSearch[value])
        showResults(cacheSearch[value], id)
    else {
        fetch("/search/api?q=" + value)
            .then(response => response.json())
            .then(data => {
                cacheSearch[value] = data
                showResults(data, id)
            })
    }
}

function showResults(results, id) {
    const resultDiv = document.getElementById(`result-${id}`)
    if(!resultDiv)
        return

    resultDiv.innerHTML = ""

    let ul = document.createElement("ul")
    ul.setAttribute("id", `playerList-${id}`)

    for(const type of ["maps", "players"]) {
        results[type].forEach(item => {
            let li = document.createElement("li")
            let p = document.createElement("p")
            let a = document.createElement("a")
            let span = document.createElement("span")

            span.classList.add("right")

            if(type == "maps") {
                a.setAttribute("href", "/map/" + encodeURIComponent(item.map.map))
                span.textContent = `${item.map.server}`
                a.textContent = item.map.map
            }
            else {
                a.setAttribute("href", "/player/" + encodeURIComponent(item.name))
                span.textContent = `${item.points} points`
                a.textContent = item.name
            }
        

            p.appendChild(a)
            p.appendChild(span)
            
            li.appendChild(p)
            ul.appendChild(li)
        })
        resultDiv.appendChild(ul)
    }
}

function onKeydown(event) {
    const id = event.target.id.split("-")[1]
    let playerList = document.getElementById(`playerList-${id}`)

    if(!playerList)
        return

    const selectedItemID = `selected-${id}`
    const selectedItem = playerList.querySelector("." + selectedItemID)

    if(event.key === "ArrowDown") {
        if(selectedItem && selectedItem.nextSibling) {
            selectedItem.nextSibling.classList.add(selectedItemID)
            selectedItem.classList.remove(selectedItemID)
        } else {
            playerList.firstChild.classList.add(selectedItemID)
            playerList.lastChild.classList.remove(selectedItemID)
        }
    } else if (event.key === "ArrowUp") {
        if(selectedItem && selectedItem.previousSibling) {
            selectedItem.previousSibling.classList.add(selectedItemID)
            selectedItem.classList.remove(selectedItemID)
        } else {
            playerList.lastChild.classList.add(selectedItemID)
            playerList.firstChild.classList.remove(selectedItemID)
        }
    } else if (event.key === "Enter") {
        if(playerList.firstChild && playerList.firstChild == playerList.lastChild) {
            event.preventDefault()
            window.location.href = playerList.firstChild.firstChild.firstChild.getAttribute("href")
        }
        if(selectedItem) {
            event.preventDefault()
            window.location.href = selectedItem.firstChild.firstChild.getAttribute("href")
        }
        if(playerList.childElementCount === 0) {
            event.preventDefault()
            window.location.href = "/player/" + encodeURIComponent(inputNav.value)
        }
    }
}
