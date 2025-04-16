function sortPosts() {
    const postList = document.querySelector('.post-list');
    if (!postList) return;
    const posts = Array.from(postList.children);
    posts.sort((a, b) => {
        const aDate = new Date(a.getAttribute('data-date'));
        const bDate = new Date(b.getAttribute('data-date'));
        return bDate - aDate;
    });
    postList.innerHTML = '';
    posts.forEach(post => {
        postList.appendChild(post);
    });
}

function filterPosts() {
    const tagQuery = new URLSearchParams(window.location.search).get('tag');
    let displayCount = 0;
    for (const post of document.querySelectorAll('.post-list li')) {
        const tags = post.getAttribute('data-tags').split(',');
        if (tagQuery && !tags.includes(tagQuery)) {
            post.style.display = 'none';
        } else {
            post.style.display = 'block';
            displayCount++;
        }
    }
    if (displayCount === 0) {
        const postList = document.querySelector('.post-list');
        if (!postList) return;
        const noPostsMessage = document.createElement('p');
        noPostsMessage.textContent = 'No posts found.';
        postList.appendChild(noPostsMessage);
    }
}

document.addEventListener('DOMContentLoaded', function () {
    sortPosts();
    filterPosts();
});