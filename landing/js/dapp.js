document.addEventListener('DOMContentLoaded', () => {
    // --- DOM Elements ---
    const elements = {
        themeToggle: document.getElementById('theme-toggle'),
        mobileMenuToggle: document.getElementById('mobile-menu-toggle'),
        mobileMenu: document.getElementById('mobile-menu'),
        contractFile: document.getElementById('contract-file'),
        network: document.getElementById('network'),
        deployContractBtn: document.getElementById('deploy-contract'),
        contractAddress: document.getElementById('contract-address'),
        functionName: document.getElementById('function-name'),
        functionParams: document.getElementById('function-params'),
        callNetwork: document.getElementById('call-network'),
        callFunctionBtn: document.getElementById('call-function'),
        deployResult: document.getElementById('deploy-result'),
        callResult: document.getElementById('call-result'),
    };

    // --- API Configuration ---
    const API_BASE = '/api/v1';
    const ENDPOINTS = {
        upload: `${API_BASE}/upload`,
        deploy: `${API_BASE}/deploy`,
        call: `${API_BASE}/call`,
    };

    // --- Functions ---
    const apiRequest = async (url, options = {}) => {
        try {
            const response = await fetch(url, options);
            if (!response.ok) {
                const errorData = await response.json().catch(() => ({ message: 'An unknown error occurred' }));
                throw new Error(errorData.message || `HTTP error! status: ${response.status}`);
            }
            return response.json();
        } catch (error) {
            console.error('API Request Failed:', error);
            showNotification(`Error: ${error.message}`, 'error');
            throw error;
        }
    };

    const showNotification = (message, type = 'info') => {
        // Create notification element
        const notification = document.createElement('div');
        notification.className = `notification notification-${type}`;
        notification.textContent = message;
        
        // Add styles
        notification.style.position = 'fixed';
        notification.style.top = '20px';
        notification.style.right = '20px';
        notification.style.padding = '1rem';
        notification.style.borderRadius = '8px';
        notification.style.color = 'white';
        notification.style.zIndex = '3000';
        notification.style.boxShadow = '0 4px 12px rgba(0,0,0,0.15)';
        notification.style.transform = 'translateX(100%)';
        notification.style.transition = 'transform 0.3s ease';
        
        if (type === 'error') {
            notification.style.backgroundColor = '#EF4444';
        } else if (type === 'success') {
            notification.style.backgroundColor = '#10B981';
        } else {
            notification.style.backgroundColor = '#4F46E5';
        }
        
        // Add to document
        document.body.appendChild(notification);
        
        // Animate in
        setTimeout(() => {
            notification.style.transform = 'translateX(0)';
        }, 10);
        
        // Remove after delay
        setTimeout(() => {
            notification.style.transform = 'translateX(100%)';
            setTimeout(() => {
                if (notification.parentNode) {
                    notification.parentNode.removeChild(notification);
                }
            }, 300);
        }, 5000);
    };

    const deployContract = async () => {
        const file = elements.contractFile.files[0];
        if (!file) {
            showNotification('Please select a contract file.', 'error');
            return;
        }

        const network = elements.network.value;
        const formData = new FormData();
        formData.append('file', file);

        try {
            const uploadResponse = await fetch(ENDPOINTS.upload, {
                method: 'POST',
                body: formData,
            });

            if (!uploadResponse.ok) {
                throw new Error('File upload failed');
            }

            const contract_path = await uploadResponse.text();

            const deployData = {
                contract_path,
                network,
            };

            const result = await apiRequest(ENDPOINTS.deploy, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(deployData),
            });
            
            elements.deployResult.innerHTML = `
                <div class="result-success">
                    <p><strong>Contract deployed successfully!</strong></p>
                    <p>Contract Address: <span class="tx-address">${result.contract_address}</span></p>
                </div>
            `;
            
            // Automatically fill the contract address in the interaction section
            elements.contractAddress.value = result.contract_address;
            
            showNotification('Contract deployed successfully!', 'success');
        } catch (error) {
            elements.deployResult.innerHTML = `
                <div class="result-error">
                    <p><strong>Deployment failed:</strong></p>
                    <p>${error.message}</p>
                </div>
            `;
        }
    };

    const callFunction = async () => {
        const contractAddress = elements.contractAddress.value;
        const functionName = elements.functionName.value;
        const network = elements.callNetwork.value;
        
        if (!contractAddress || !functionName) {
            showNotification('Please enter contract address and function name.', 'error');
            return;
        }
        
        let params = null;
        if (elements.functionParams.value) {
            try {
                params = JSON.parse(elements.functionParams.value);
            } catch (e) {
                showNotification('Invalid JSON in function parameters.', 'error');
                return;
            }
        }
        
        const callData = {
            contract_address: contractAddress,
            function_name: functionName,
            network: network
        };
        
        if (params) {
            callData.params = params;
        }
        
        try {
            const result = await apiRequest(ENDPOINTS.call, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(callData),
            });
            
            elements.callResult.innerHTML = `
                <div class="result-success">
                    <p><strong>Function call successful!</strong></p>
                    <pre>${JSON.stringify(result, null, 2)}</pre>
                </div>
            `;
            
            showNotification('Function called successfully!', 'success');
        } catch (error) {
            elements.callResult.innerHTML = `
                <div class="result-error">
                    <p><strong>Function call failed:</strong></p>
                    <p>${error.message}</p>
                </div>
            `;
        }
    };

    const toggleTheme = () => {
        document.body.classList.toggle('dark-mode');
        localStorage.setItem('theme', document.body.classList.contains('dark-mode') ? 'dark' : 'light');
        elements.themeToggle.innerHTML = `<i class="fas ${document.body.classList.contains('dark-mode') ? 'fa-sun' : 'fa-moon'}"></i>`;
    };

    const toggleMobileMenu = () => {
        elements.mobileMenu.style.display = elements.mobileMenu.style.display === 'block' ? 'none' : 'block';
    };

    const closeMobileMenu = () => {
        elements.mobileMenu.style.display = 'none';
    };

    // --- Event Listeners ---
    elements.contractFile.addEventListener('change', () => {
        elements.deployContractBtn.disabled = !elements.contractFile.files.length;
    });
    
    elements.contractAddress.addEventListener('input', () => {
        elements.callFunctionBtn.disabled = !elements.contractAddress.value || !elements.functionName.value;
    });
    
    elements.functionName.addEventListener('input', () => {
        elements.callFunctionBtn.disabled = !elements.contractAddress.value || !elements.functionName.value;
    });

    if (elements.deployContractBtn) {
        elements.deployContractBtn.addEventListener('click', deployContract);
    }
    
    if (elements.callFunctionBtn) {
        elements.callFunctionBtn.addEventListener('click', callFunction);
    }
    
    if (elements.themeToggle) {
        if (localStorage.getItem('theme') === 'dark') {
            document.body.classList.add('dark-mode');
            elements.themeToggle.innerHTML = '<i class="fas fa-sun"></i>';
        }
        elements.themeToggle.addEventListener('click', toggleTheme);
    }

    // --- Mobile Menu ---
    if (elements.mobileMenuToggle) {
        elements.mobileMenuToggle.addEventListener('click', toggleMobileMenu);
    }

    // Close mobile menu when clicking on links
    const mobileMenuLinks = document.querySelectorAll('#mobile-menu a');
    mobileMenuLinks.forEach(link => {
        link.addEventListener('click', closeMobileMenu);
    });

    // Close mobile menu when clicking outside
    document.addEventListener('click', (event) => {
        if (elements.mobileMenu && elements.mobileMenu.style.display === 'block' && 
            !elements.mobileMenu.contains(event.target) && 
            !elements.mobileMenuToggle.contains(event.target)) {
            closeMobileMenu();
        }
    });
});