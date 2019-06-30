
function createInputColumn(name, type, use_value) {
    let col = document.createElement("span");
    col.className = "td";
    let input = document.createElement("input");
    input.type = type;
    if (use_value !== true) {
        input.name = name;
    } else {
        input.value = name;
    }
    col.appendChild(input);

    return col;
}

function doAddRow(action_path, column_specs, table_id) {
    let add_row = document.createElement("form");
    add_row.className = "tr";
    add_row.action = action_path;
    add_row.method = "post";

    for (i = 0; i < column_specs.length; i++) {
        let col_spec = column_specs[i]
        let dom_col = createInputColumn(col_spec[0], col_spec[1])
        add_row.appendChild(dom_col);
    }
    let submit_col = createInputColumn("Add", "submit", true);
    add_row.appendChild(submit_col);

    let table_div = document.getElementById(table_id);
    table_div.appendChild(add_row);
}