import './globals.css'

export const metadata = {
    title: 'Pathfinder',
    description: 'Generated by create next app',
    icons: {
        icon: [
            { url: '/favicon-32x32.png', sizes: "32x32" },
            { url: '/favicon-16x16.png', sizes: "16x16" },
            { url: '/favicon.ico' }
        ],
        apple: [
            { url: '/apple-touch-icon.png' }
        ],
        other: [
            {
                rel: 'apple-touch-icon',
                url: '/apple-touch-icon.png',
            }
        ]
    },
    manifest: '/site.webmanifest',
    twitter: {
        card: 'summary_large_image',
        title: 'Pathfinder Guidance',
        description: 'College guidance reimagined',
        siteId: '',
        creator: '@dsagllups',
        creatorId: '',
        images: ['/pflogo.png'],
    },
}

export default function RootLayout({
    children,
}: {
    children: React.ReactNode
}) {
    return (
        <html lang="en">
            <body>{children}</body>
        </html>
    )
}
