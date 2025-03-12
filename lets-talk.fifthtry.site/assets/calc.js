function updateSize() {
            const width = window.innerWidth;
            const height = window.innerHeight;
            
            document.getElementById('width').textContent = width;
            document.getElementById('height').textContent = height;

            localStorage.setItem('pageWidth', width);
            localStorage.setItem('pageHeight', height);
        }

        function loadStoredSize() {
            const storedWidth = localStorage.getItem('pageWidth');
            const storedHeight = localStorage.getItem('pageHeight');

            if (storedWidth && storedHeight) {
                document.getElementById('width').textContent = storedWidth;
                document.getElementById('height').textContent = storedHeight;
            }
        }

        window.addEventListener('resize', updateSize);
        window.addEventListener('load', () => {
            loadStoredSize();
            updateSize();
        });