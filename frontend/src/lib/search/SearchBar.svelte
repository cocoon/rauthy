<script>
    import {run} from 'svelte/legacy';

    import IconMagnify from "$lib/icons/IconMagnify.svelte";
    import {onMount} from "svelte";
    import Tooltip from "../Tooltip.svelte";
    import {getKey} from "../utils/helpers";
    import IconBackspace from "$lib/icons/IconBackspace.svelte";
    import {getSearch} from "../../utils/dataFetchingAdmin.js";
    import {SERVER_SIDE_SEARCH_THRES} from "../../utils/constants.js";


    /**
     * @typedef {Object} Props
     * @property {any} [items]
     * @property {any} resItems
     * @property {any} [options]
     * @property {string} [useServerSideIdx]
     * @property {boolean} [isSearchFiltered]
     * @property {string} [search]
     */

    /** @type {Props} */
    let {
        items = $bindable([]),
        resItems = $bindable(),
        options = [],
        useServerSideIdx = $bindable(''),
        isSearchFiltered = $bindable(false),
        search = $bindable('')
    } = $props();
    let selected = $state('');
    let callback;

    onMount(() => {
        if (options.length > 0) {
            selected = options[0].label;
            extractCallback();
        }
    });


    function extractCallback() {
        for (let opt of options) {
            if (opt.label === selected) {
                callback = opt.callback;
                break;
            }
        }

        if (!callback) {
            console.error('Could not find a valid callback function in search options for label ' + selected);
        }
    }

    function del() {
        search = '';
    }

    function filerItems() {
        if (search.length < 2) {
            resItems = items;
            isSearchFiltered = false;
            return;
        }

        resItems = [...items.filter(i => {
            // This switch is a bit more annoying to maintain, but we can set a more strict CSP without `eval`
            if (options.length > 0) {
                return callback(i, search);
            } else {
                return i.toLowerCase().includes(search) || i === search;
            }
        })];
        isSearchFiltered = true;
    }

    async function filerItemsServerSide() {
        if (search.length < SERVER_SIDE_SEARCH_THRES) {
            // skipping server side search below 3 chars
            resItems = items;
            isSearchFiltered = false;
            return;
        }
        isSearchFiltered = true;

        const idx = selected.replaceAll('-', '').replaceAll(' ', '').toLowerCase();
        let res = await getSearch(useServerSideIdx, idx, search);
        if (res.ok) {
            resItems = await res.json();
        } else {
            // should never happen ...
            console.error(res);
        }
    }

    run(() => {
        if (selected) {
            extractCallback();
        }
    });
    run(() => {
        if (!search) {
            resItems = items;
            isSearchFiltered = false;
        } else if (useServerSideIdx) {
            filerItemsServerSide();
        } else {
            filerItems();
        }
    });
</script>

<div class="container">
    {#if options.length > 1}
        <Tooltip text="Search by" yOffset={-30}>
            <select class="opts font-label" bind:value={selected}>
                {#each options as opt}
                    <option value={opt.label}>{opt.label}</option>
                {/each}
            </select>
        </Tooltip>
    {/if}

    <div class="inputBar">
        <input
                class="input"
                type="text"
                name={getKey()}
                bind:value={search}
                placeholder="Search"
                autocomplete="off"
        />
        <div class="magnify">
            <IconMagnify width={20}/>
        </div>
    </div>

    <div class="backWrap">
        <div
                role="button"
                tabindex="0"
                class="back"
                onclick={del}
                onkeypress={del}
        >
            <IconBackspace/>
        </div>
    </div>
</div>

<style>
    .back {
        position: absolute;
        top: -12px;
        right: 8px;
        cursor: pointer;
        color: var(--col-gmid)
    }

    .backWrap {
        position: relative;
    }

    .opts {
        margin-right: 15px;
    }

    .container {
        width: 100%;
        display: flex;
        align-items: center;
    }

    input {
        padding: 5px 30px 5px 25px;
        border: 1px solid hsl(var(--bg-high));
        border-radius: 3px;
        color: hsl(var(--text));
        font-size: 1.05rem;
        outline: none;
        box-shadow: 1px 1px 2px hsl(var(--bg-high));
    }

    input:focus {
        border: 1px solid hsl(var(--accent));
    }

    .inputBar {
        position: relative;
    }

    .magnify {
        position: absolute;
        top: 12px;
        left: 5px;
    }

    select {
        height: 2.13rem;
        padding-top: .2rem;
        padding-left: .5rem;
        background: transparent;
        color: hsl(var(--text));
        font-size: 1.05rem;
        border-radius: 3px;
        cursor: pointer;
        border: 1px solid hsl(var(--bg-high));
        box-shadow: 1px 1px 2px hsl(var(--bg-high));
    }
</style>
