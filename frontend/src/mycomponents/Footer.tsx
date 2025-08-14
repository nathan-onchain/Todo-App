import { Shield } from "lucide-react";

const Footer = () => {
  return (
    <footer className="py-12 bg-muted border-t border-border">
      <div className="container mx-auto px-4">
        <div className="max-w-4xl mx-auto">
          <div className="flex flex-col md:flex-row justify-between items-center">
            <div className="flex items-center space-x-2 mb-4 md:mb-0">
              <div className="w-8 h-8 bg-primary rounded-lg flex items-center justify-center">
                <span className="text-primary-foreground font-bold text-lg">T</span>
              </div>
              <span className="text-xl font-semibold text-foreground">TaskTonic</span>
            </div>
            
            <div className="flex items-center space-x-6 mb-4 md:mb-0">
              <a href="/privacy" className="text-muted-foreground hover:text-foreground transition-colors text-sm">
                Privacy Policy
              </a>
              <a href="/terms" className="text-muted-foreground hover:text-foreground transition-colors text-sm">
                Terms of Service
              </a>
              <a href="/support" className="text-muted-foreground hover:text-foreground transition-colors text-sm">
                Support
              </a>
            </div>
          </div>
          
          <div className="border-t border-border mt-8 pt-8 flex flex-col md:flex-row justify-between items-center text-sm text-muted-foreground">
            <div className="flex items-center space-x-4 mb-4 md:mb-0">
              <div className="flex items-center">
                <Shield className="w-4 h-4 mr-2 text-green-500" />
                <span>HTTPS Secure Connection</span>
              </div>
              <span>•</span>
              <span>GDPR Compliant</span>
            </div>
            
            <p>© 2024 TaskTonic. All rights reserved.</p>
          </div>
        </div>
      </div>
    </footer>
  );
};

export default Footer;