var basepath = "";

let importquery = "";
var loadfailed = false;

// set global load failed .. TODO: replace with html element id
const params = new URL(document.location.toString()).searchParams;
if (params.get('sig') === null) {
    loadfailed = true;
}

var elementsToAppend = [];
function appendElement(type, attrs) {
    elementsToAppend.push({
        'type': type,
        'attrs': attrs,
    });
}

window.onload = (async function () {
    const body = document.getElementsByTagName("head")[0];
    const params = new URL(document.location.toString()).searchParams;

    // keys to forward to the js and css scripts
    const querykeys = [
        "sv", // signedVersion
        "sr", // signedResource
        "rscc", // Cache-Control
        "rscd", // Content-Disposition
        "rsce", // Content-Encoding
        "rscl", // Content-Language
        "rsct", // Content-Type
        "tn", // table storage
        "st", // signedStart
        "se", // signedExpiry
        "sp", // signedPermissions1
        "spk", // startPk2
        "srk", // startRk2
        "epk", // endPk2
        "erk", // endRk2
        "sip", // signedIp
        "spr", // signedProtocol
        "sdd", // signedDirectoryDepth
        "si", // signedIdentifier
        "ses", // signedEncryptionScope
        "sig", // signature
    ];

    function appendquery(params, key) {
        return `${key}=${encodeURIComponent(params.get(key))}`;
    }

    function createquery(keys, params) {
        let query = "";
        var first = true;
        for (const key of keys) {
            if (!params.has(key)) {
                continue;
            }

            if (first) {
                first = false;
                query += "?";
            } else {
                query += "&";
            }
            query += appendquery(params, key);
        }
        return query;
    }

    importquery = createquery(querykeys, params);
    let basepath = "";

    function appendBasePath(href) {
        if (!basepath) {
            return `${href}${importquery}`;
        }
        return `${basepath}/${href}${importquery}`;
    }

    function appendElementLoad(type, attrs) {
        // TODO: append to queue and execute after html is loaded ..
        // update href / src by appending basepath
        if ('href' in attrs) {
            attrs['href'] = appendBasePath(attrs['href'])
        }
        if ('src' in attrs) {
            attrs['src'] = appendBasePath(attrs['src'])
        }

        console.debug(`load <${type} href='${attrs['href']}' src='${attrs['src']}' />`);
        if (loadfailed) {
            console.log('skip load, no sig provided');
            return;
        }
        var el = document.createElement(type);
        for (const [key, value] of Object.entries(attrs)) {
            el.setAttribute(key, value);
        }
        body.append(el);
    }

    async function loadConfig() {
        config_file = "config.json";
        let json = {};
        const response = await fetch(config_file);
        if (!response.ok) {
            console.info(`no '${config_file}' file provided`);
            return json;
        }
        try {
            json = await response.json();
            console.debug(`loaded config ${JSON.stringify(json)}`);
        } catch {
            console.info(`'${config_file}' file received is invalid`);
        }
        return json;
    }

    config = await loadConfig();
    if (config.basepath) {
        basepath = config.basepath;
    }


    for (const { type, attrs } of elementsToAppend) {
        appendElementLoad(type, attrs);
    }
});