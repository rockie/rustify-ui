const form = document.getElementById("cta-test-form");
const fields = [...form.querySelectorAll("input[required]")];
const button = form.querySelector("button[type=submit]");
const status = document.getElementById("cta-status");

function validate(field) {
    const valid = field.validity.valid;
    field.setAttribute("aria-invalid", String(!valid));
    const error = document.getElementById(`${field.id}-error`);
    error.textContent = valid ? "" : field.validity.typeMismatch
        ? "请填写有效的邮箱地址。"
        : field.name === "email" ? "请填写测试邮箱。" : "请填写测试项目名称。";
    return valid;
}

form.noValidate = true;
for (const field of fields) {
    field.addEventListener("input", () => {
        if (field.getAttribute("aria-invalid") === "true") validate(field);
        if (fields.every((input) => input.validity.valid)) status.textContent = "";
    });
}

form.addEventListener("submit", (event) => {
    for (const field of fields) field.value = field.value.trim();
    const valid = fields.map(validate).every(Boolean);
    if (!valid) {
        event.preventDefault();
        fields.find((field) => !field.validity.valid).focus();
        status.textContent = "请修正标出的内容后重试。";
        return;
    }
    button.disabled = true;
    button.textContent = "正在提交…";
    form.setAttribute("aria-busy", "true");
    status.textContent = "正在提交到本地测试接收器…";
});

window.addEventListener("pageshow", () => {
    button.disabled = false;
    button.textContent = "提交测试信息";
    form.removeAttribute("aria-busy");
    status.textContent = "";
});
