const path = require("path");
const fs = require("fs");

/** @param {string} str */
function posixify(str) {
    return str.replace(/\\/g, "/");
}

/** @param {string} dir */
function mkdirp(dir) {
    try {
        fs.mkdirSync(dir, { recursive: true });
    } catch (/** @type {any} */ e) {
        if (e.code === "EEXIST") {
            if (!fs.statSync(dir).isDirectory()) {
                throw new Error(
                    `Cannot create directory ${dir}, a file already exists at this position`
                );
            }
            return;
        }
        throw e;
    }
}

/**
 * @param {string} source
 * @param {string} target
 * @param {{
 *   filter?: (basename: string) => boolean;
 *   replace?: Record<string, string>;
 * }} opts
 */
function copy(source, target, opts = {}) {
    if (!fs.existsSync(source)) return [];

    /** @type {string[]} */
    const files = [];

    const prefix = posixify(target) + "/";

    const regex = opts.replace
        ? new RegExp(`\\b(${Object.keys(opts.replace).join("|")})\\b`, "g")
        : null;

    /**
     * @param {string} from
     * @param {string} to
     */
    function go(from, to) {
        if (opts.filter && !opts.filter(path.basename(from))) return;

        const stats = fs.statSync(from);

        if (stats.isDirectory()) {
            fs.readdirSync(from).forEach((file) => {
                go(path.join(from, file), path.join(to, file));
            });
        } else {
            mkdirp(path.dirname(to));

            if (opts.replace) {
                const data = fs.readFileSync(from, "utf-8");
                fs.writeFileSync(
                    to,
                    data.replace(
                        /** @type {RegExp} */ (regex),
                        (_match, key) =>
                            /** @type {Record<string, string>} */ (
                                opts.replace
                            )[key]
                    )
                );
            } else {
                fs.copyFileSync(from, to);
            }

            files.push(
                to === target
                    ? posixify(path.basename(to))
                    : posixify(to).replace(prefix, "")
            );
        }
    }

    go(source, target);

    return files;
}

copy("./resources", "./pkg", {
    replace: {
        VERSION: JSON.parse(fs.readFileSync("./package.json")).version + "",
    },
});
