

window.addEventListener('DOMContentLoaded', async () => {
    //下一页
    function nextPage() {
        const nextButton = document.querySelector('button.renderTarget_pager_button_right');
        if (nextButton) {
            nextButton.click();
        } else {
            console.log('未找到下一页按钮');
        }
    }

});