import { CheckCircle, Smartphone, Users, Zap, Shield, Calendar } from "lucide-react";

const benefits = [
  {
    icon: CheckCircle,
    title: "Never Miss a Deadline",
    description: "Smart reminders and notifications keep you on track with automatic alerts"
  },
  {
    icon: Smartphone,
    title: "Access from Any Device", 
    description: "Seamlessly sync across desktop, mobile, and tablet with real-time updates"
  },
  {
    icon: Users,
    title: "Collaborate in Real Time",
    description: "Share projects and tasks with your team for effortless collaboration"
  },
  {
    icon: Zap,
    title: "Lightning Fast Performance",
    description: "Optimized for speed with instant loading and smooth interactions"
  },
  {
    icon: Shield,
    title: "Bank-Level Security",
    description: "Your data is protected with enterprise-grade encryption and privacy"
  },
  {
    icon: Calendar,
    title: "Smart Scheduling",
    description: "AI-powered suggestions help you organize tasks by priority and timeline"
  }
];

const Benefits = () => {
  return (
    <section id="features" className="py-20 bg-background">
      <div className="container mx-auto px-4">
        <div className="text-center mb-16 animate-fade-in">
          <h2 className="text-3xl md:text-4xl lg:text-5xl font-bold text-foreground mb-6">
            Everything You Need to{" "}
            <span className="bg-text-gradient bg-clip-text text-transparent">
              Stay Productive
            </span>
          </h2>
          <p className="text-xl text-muted-foreground max-w-3xl mx-auto">
            TaskTonic combines powerful features with beautiful design to help you 
            accomplish more while feeling less overwhelmed.
          </p>
        </div>
        
        <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
          {benefits.map((benefit, index) => {
            const Icon = benefit.icon;
            return (
              <div 
                key={index}
                className="group p-8 bg-card rounded-2xl shadow-card border border-border hover:shadow-lg transition-all duration-300 animate-fade-in"
                style={{ animationDelay: `${index * 0.1}s` }}
              >
                <div className="w-12 h-12 bg-primary/10 rounded-xl flex items-center justify-center mb-6 group-hover:bg-primary/20 transition-colors">
                  <Icon className="w-6 h-6 text-primary" />
                </div>
                <h3 className="text-xl font-semibold text-foreground mb-3">
                  {benefit.title}
                </h3>
                <p className="text-muted-foreground leading-relaxed">
                  {benefit.description}
                </p>
              </div>
            );
          })}
        </div>
      </div>
    </section>
  );
};

export default Benefits;